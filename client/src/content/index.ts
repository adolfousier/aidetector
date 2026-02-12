import type { Platform, AnalyzeResponse, ExtensionSettings } from "../shared/types";
import type { PostData } from "./platforms/twitter";
import { observeFeed } from "./observer";
import { injectLoadingBadge, updateBadge, injectErrorBadge } from "./inject";

type PlatformModule = {
  extractPosts: () => PostData[];
  getFeedContainer: () => HTMLElement | null;
};

function detectPlatform(): Platform | null {
  const host = window.location.hostname;
  if (host === "twitter.com" || host === "x.com") return "twitter";
  if (host === "www.instagram.com" || host === "instagram.com") return "instagram";
  if (host === "www.linkedin.com" || host === "linkedin.com") return "linkedin";
  return null;
}

async function loadPlatformModule(
  platform: Platform
): Promise<PlatformModule> {
  switch (platform) {
    case "twitter":
      return import("./platforms/twitter");
    case "instagram":
      return import("./platforms/instagram");
    case "linkedin":
      return import("./platforms/linkedin");
  }
}

function bgLog(msg: string): void {
  chrome.runtime.sendMessage({ type: "LOG", payload: msg });
}

function sendAnalyzeMessage(
  text: string,
  platform: Platform,
  postId: string,
  author: string
): Promise<AnalyzeResponse> {
  return new Promise((resolve, reject) => {
    chrome.runtime.sendMessage(
      {
        type: "ANALYZE",
        payload: { content: text, platform, post_id: postId, author },
      },
      (response) => {
        if (chrome.runtime.lastError) {
          reject(new Error(chrome.runtime.lastError.message));
          return;
        }
        if (response?.success) {
          resolve(response.data as AnalyzeResponse);
        } else {
          reject(new Error(response?.error || "Analysis failed"));
        }
      }
    );
  });
}

function getSettings(): Promise<ExtensionSettings> {
  return new Promise((resolve, reject) => {
    chrome.runtime.sendMessage({ type: "GET_SETTINGS" }, (response) => {
      if (chrome.runtime.lastError) {
        reject(new Error(chrome.runtime.lastError.message));
        return;
      }
      if (response?.success) {
        resolve(response.data as ExtensionSettings);
      } else {
        reject(new Error("Failed to get settings"));
      }
    });
  });
}

async function processPost(post: PostData): Promise<void> {
  // Mark as processed
  post.element.dataset.aidProcessed = "true";

  // Inject loading badge
  const host = injectLoadingBadge(post.injectTarget);

  try {
    const result = await sendAnalyzeMessage(
      post.text,
      post.platform,
      post.postId,
      post.author
    );
    updateBadge(host, result);
  } catch (err) {
    console.warn("[AI Detector] Analysis failed:", err);
    injectErrorBadge(host);
  }
}

async function scanAndProcess(platformModule: PlatformModule): Promise<void> {
  const posts = platformModule.extractPosts();
  bgLog(`Found ${posts.length} new posts to analyze`);
  const batch = posts.slice(0, 5);
  await Promise.allSettled(batch.map(processPost));
}

async function init() {
  const platform = detectPlatform();
  bgLog(`Platform detected: ${platform} (host: ${window.location.hostname})`);
  if (!platform) return;

  // Check settings
  let settings: ExtensionSettings;
  try {
    settings = await getSettings();
  } catch (err) {
    bgLog(`Could not load settings, aborting: ${err}`);
    return;
  }

  if (!settings.enabled) {
    bgLog("Extension disabled in settings");
    return;
  }
  if (!settings.enabledPlatforms[platform]) {
    bgLog(`Platform ${platform} disabled in settings`);
    return;
  }

  bgLog(`Active on ${platform}`);

  const platformModule = await loadPlatformModule(platform);

  // Listen for rescan requests from popup
  chrome.runtime.onMessage.addListener((message) => {
    if (message?.type === "RESCAN") {
      bgLog("Rescan triggered from popup");
      scanAndProcess(platformModule);
    }
  });

  // Initial scan after a short delay to let the page load
  setTimeout(() => {
    bgLog("Running initial scan...");
    scanAndProcess(platformModule);
  }, 2000);

  // Watch for new content
  const container = platformModule.getFeedContainer();
  bgLog(`Feed container: ${container ? container.tagName + "." + container.className.slice(0, 60) : "NOT FOUND"}`);
  if (container) {
    observeFeed(container, () => scanAndProcess(platformModule));
  } else {
    bgLog("No feed container, will retry every 2s for 30s...");
    const retryInterval = setInterval(() => {
      const c = platformModule.getFeedContainer();
      if (c) {
        bgLog(`Feed container found on retry: ${c.tagName}.${c.className.slice(0, 60)}`);
        clearInterval(retryInterval);
        observeFeed(c, () => scanAndProcess(platformModule));
      }
    }, 2000);

    setTimeout(() => clearInterval(retryInterval), 30000);
  }
}

init();
