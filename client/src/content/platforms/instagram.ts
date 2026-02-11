import type { PostData } from "./twitter";

export function extractPosts(): PostData[] {
  const posts: PostData[] = [];
  const articles = document.querySelectorAll<HTMLElement>("article");

  for (const article of articles) {
    if (article.dataset.aidProcessed) continue;

    // Instagram caption text is typically in spans within the article
    const captionSpans = article.querySelectorAll<HTMLSpanElement>(
      'span[dir="auto"]'
    );
    let text = "";
    for (const span of captionSpans) {
      const t = span.textContent?.trim() || "";
      if (t.length > text.length) {
        text = t; // Use the longest text span as caption
      }
    }

    if (text.length < 10) continue;

    // Find the username
    const authorEl = article.querySelector<HTMLAnchorElement>(
      'header a, a[role="link"]'
    );
    const author =
      authorEl?.textContent?.trim() ||
      authorEl?.getAttribute("href")?.replace(/\//g, "") ||
      "unknown";

    const postId = `ig-${Date.now()}-${posts.length}`;

    // Inject near the header
    const header = article.querySelector<HTMLElement>("header");
    const injectTarget = header || article;

    posts.push({
      element: article,
      text,
      postId,
      author,
      platform: "instagram",
      injectTarget,
    });
  }

  return posts;
}

export function getFeedContainer(): HTMLElement | null {
  return (
    document.querySelector<HTMLElement>("main") ||
    document.querySelector<HTMLElement>('[role="main"]') ||
    null
  );
}
