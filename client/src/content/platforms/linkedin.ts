import type { PostData } from "./twitter";

export function extractPosts(): PostData[] {
  const posts: PostData[] = [];

  // LinkedIn 2025+ uses obfuscated class names.
  // Feed posts are div[role="listitem"] inside the main feed area.
  const listItems = document.querySelectorAll<HTMLElement>('div[role="listitem"]');

  for (const item of listItems) {
    if (item.dataset.aidProcessed) continue;

    // Skip items with very little text (not real posts)
    const fullText = (item.textContent || "").trim();
    if (fullText.length < 50) continue;

    const text = extractText(item);
    if (text.length < 10) continue;

    const author = extractAuthor(item);
    const postId = derivePostId(item, author, text);
    const injectTarget = findInjectTarget(item);

    posts.push({
      element: item,
      text,
      postId,
      author,
      platform: "linkedin",
      injectTarget,
    });
  }

  return posts;
}

function extractText(postEl: HTMLElement): string {
  // Strategy 1: Find span[dir="ltr"] with substantial text (LinkedIn still uses dir for text)
  const ltrSpans = postEl.querySelectorAll<HTMLSpanElement>('span[dir="ltr"]');
  if (ltrSpans.length > 0) {
    const parts: string[] = [];
    for (const span of ltrSpans) {
      const t = (span.textContent || "").trim();
      // Skip short UI labels and author names (they tend to be short)
      if (t.length > 20) parts.push(t);
    }
    const combined = parts.join(" ").trim();
    if (combined.length >= 30) return combined;
  }

  // Strategy 2: Find the largest text block that isn't a link or small UI element
  // Walk all text-bearing elements and find the one with the most content
  const candidates: { el: Element; text: string }[] = [];
  const walker = document.createTreeWalker(postEl, NodeFilter.SHOW_ELEMENT);
  let node: Node | null = walker.currentNode;
  while ((node = walker.nextNode())) {
    const el = node as Element;
    // Skip link elements (these are typically author names, hashtags, etc.)
    if (el.tagName === "A") continue;
    // Look for leaf-ish text containers
    if (el.children.length > 5) continue;
    const t = (el.textContent || "").trim();
    if (t.length > 50) {
      candidates.push({ el, text: t });
    }
  }

  // Sort by text length descending, pick the best that looks like post content
  candidates.sort((a, b) => b.text.length - a.text.length);

  for (const c of candidates) {
    // Skip if this is clearly the whole post container (too much text including UI)
    if (c.text.length > 2000) continue;
    // Skip if it starts with common UI patterns
    if (c.text.startsWith("Like") || c.text.startsWith("Comment")) continue;
    return c.text;
  }

  // Strategy 3: fallback — get all <p> or <span> text
  const spans = postEl.querySelectorAll("span, p");
  let longest = "";
  for (const s of spans) {
    const t = (s.textContent || "").trim();
    if (t.length > longest.length && t.length < 2000 && s.children.length < 3) {
      longest = t;
    }
  }

  return longest;
}

function extractAuthor(postEl: HTMLElement): string {
  // Find profile links — LinkedIn always uses /in/username or /company/name
  const profileLink = postEl.querySelector<HTMLAnchorElement>(
    'a[href*="/in/"], a[href*="/company/"]'
  );
  if (profileLink) {
    // Try getting visible text first
    const linkText = (profileLink.textContent || "").trim();
    if (linkText.length > 0 && linkText.length < 100) return linkText;

    // Fallback: extract from href
    const href = profileLink.getAttribute("href") || "";
    const match = href.match(/\/in\/([^/?]+)|\/company\/([^/?]+)/);
    if (match) return match[1] || match[2] || "unknown";
  }

  return "unknown";
}

function derivePostId(postEl: HTMLElement, author: string, text: string): string {
  // Check for any data attributes that could serve as ID
  const urn = postEl.getAttribute("data-urn") || postEl.getAttribute("data-id");
  if (urn) return urn;

  // Derive a stable ID from author + text prefix
  const prefix = text.slice(0, 60).replace(/\s+/g, "_");
  return `li-${author}-${prefix}`;
}

function findInjectTarget(postEl: HTMLElement): HTMLElement {
  // Find a good place to inject the badge
  // Try to find social action buttons area (like, comment, share)
  // These typically contain buttons — find the container with multiple buttons
  const buttons = postEl.querySelectorAll("button");
  if (buttons.length >= 3) {
    // The container of the last group of buttons is likely the social actions bar
    const lastButton = buttons[buttons.length - 1];
    if (lastButton.parentElement) return lastButton.parentElement;
  }

  // Fallback: find the first profile link's container (header area)
  const profileLink = postEl.querySelector('a[href*="/in/"], a[href*="/company/"]');
  if (profileLink?.parentElement) return profileLink.parentElement;

  return postEl;
}

export function getFeedContainer(): HTMLElement | null {
  // Find the parent of role="listitem" elements — that's the feed container
  const firstItem = document.querySelector('div[role="listitem"]');
  if (firstItem?.parentElement) return firstItem.parentElement;

  // Fallback
  return (
    document.querySelector<HTMLElement>('[role="main"]') ||
    document.querySelector<HTMLElement>("main") ||
    null
  );
}
