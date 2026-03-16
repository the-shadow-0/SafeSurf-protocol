// SafeSurf Browser Extension - Background Script (Stub)

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === "ANALYZE_PAGE") {
    // Send to local safe_surfd daemon via REST
    fetch("http://127.0.0.1:3000/content/risk", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        url: sender.tab.url,
        html: request.html,
        headers: {}
      })
    })
    .then(res => res.json())
    .then(report => {
      chrome.action.setBadgeText({ text: report.score > 0.5 ? "!" : "ok" });
      sendResponse(report);
    })
    .catch(err => console.error("SafeSurf Daemon unreachable:", err));
    return true; // async
  }
});
