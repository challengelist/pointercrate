import {
  initializeRecordSubmitter,
  StatsViewer,
} from "./modules/demonlist.mjs";

$(document).ready(function () {
  initializePositionChart();
  initializeRecordSubmitter();

  window.statsViewer = new StatsViewer(document.getElementById("statsviewer"));

  document
    .getElementById("show-stats-viewer")
    .addEventListener("click", () => window.statsViewer.initialize());
});

function initializePositionChart() {
  return true;
}
