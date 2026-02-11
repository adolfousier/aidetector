import type { Platform } from "../../shared/types";

export interface PostData {
  element: HTMLElement;
  text: string;
  postId: string;
  author: string;
  platform: Platform;
  injectTarget: HTMLElement;
}

export function extractPosts(): PostData[] {
  const posts: PostData[] = [];
  const articles = document.querySelectorAll<HTMLElement>(
    'article[data-testid="tweet"]'
  );

  for (const article of articles) {
    // Skip already-processed posts
    if (article.dataset.aidProcessed) continue;

    const textEl = article.querySelector<HTMLElement>(
      '[data-testid="tweetText"]'
    );
    if (!textEl) continue;

    const text = textEl.textContent?.trim() || "";
    if (text.length < 10) continue;

    // Extract author from the user name link
    const authorEl = article.querySelector<HTMLAnchorElement>(
      '[data-testid="User-Name"] a[role="link"]'
    );
    const author =
      authorEl?.getAttribute("href")?.replace("/", "") || "unknown";

    // Try to get post ID from time link
    const timeLink = article.querySelector<HTMLAnchorElement>("time")
      ?.closest("a");
    const postId = timeLink?.getAttribute("href") || `tw-${Date.now()}`;

    // Find injection point (action buttons area)
    const actionGroup = article.querySelector<HTMLElement>(
      '[role="group"]'
    );
    const injectTarget = actionGroup || textEl;

    posts.push({
      element: article,
      text,
      postId,
      author,
      platform: "twitter",
      injectTarget,
    });
  }

  return posts;
}

export function getFeedContainer(): HTMLElement | null {
  // Main timeline container
  return (
    document.querySelector<HTMLElement>(
      '[data-testid="primaryColumn"]'
    ) ||
    document.querySelector<HTMLElement>("main") ||
    null
  );
}
