export function observeFeed(
  container: HTMLElement,
  onNewContent: () => void
): MutationObserver {
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  const observer = new MutationObserver(() => {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(onNewContent, 500);
  });

  observer.observe(container, {
    childList: true,
    subtree: true,
  });

  return observer;
}
