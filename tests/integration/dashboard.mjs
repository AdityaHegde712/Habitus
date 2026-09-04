import { spawn } from "node:child_process";
import { once } from "node:events";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import { chromium } from "playwright";

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..");
const serverUrl = "http://127.0.0.1:4173";
const today = new Date().toLocaleDateString("en-CA");

function taskIdsFor(date) {
  const isExerciseDay = [1, 3, 5].includes(new Date(`${date}T12:00:00`).getDay());
  const ids = ["meals", "sleep_7h"];

  if (isExerciseDay) {
    ids.push("exercise");
  }

  return ids.concat(["job_application", "vitamins", "leetcode_or_dsa", "surfaces_clean"]);
}

function dayView(date, checkedTaskIds) {
  const applicableTaskIds = taskIdsFor(date);

  return {
    local_date: date,
    applicable_task_ids: applicableTaskIds,
    checked_task_ids: checkedTaskIds,
    applicable_count: applicableTaskIds.length,
    completed_count: checkedTaskIds.length,
    policy_version: 1,
    updated_at_utc: "2026-09-03T12:00:00Z",
  };
}

const yesterday = new Date();
yesterday.setDate(yesterday.getDate() - 1);
const yesterdayIso = yesterday.toLocaleDateString("en-CA");
const historicalCheckedIds = taskIdsFor(yesterdayIso).slice(0, -1);

const fixture = {
  [today]: dayView(today, ["meals", "sleep_7h", "vitamins", "surfaces_clean"]),
  [yesterdayIso]: dayView(yesterdayIso, historicalCheckedIds),
};

function startPreviewServer() {
  const viteEntryPoint = resolve(repositoryRoot, "node_modules", "vite", "bin", "vite.js");
  const nodePath = process.execPath;
  const previewProcess = spawn(nodePath, [viteEntryPoint, "preview", "--host", "127.0.0.1"], {
    cwd: repositoryRoot,
    stdio: "ignore",
  });

  return previewProcess;
}

async function waitForServer() {
  for (let attempt = 0; attempt < 30; attempt += 1) {
    try {
      const response = await fetch(serverUrl);
      if (response.ok) {
        return;
      }
    } catch {
      // Vite has not started listening yet.
    }

    await new Promise((resolveDelay) => setTimeout(resolveDelay, 100));
  }

  throw new Error("Vite preview server did not start for the dashboard test.");
}

async function installTauriMock(page) {
  await page.addInitScript(({ dayFixture, calendarDate }) => {
    window.__TAURI_INTERNALS__ = {
      invoke: async (command, arguments_) => {
        if (command === "get_day") {
          return dayFixture[arguments_.localDate];
        }

        if (command === "list_calendar_days") {
          return Object.values(dayFixture).map((day) => ({
            local_date: day.local_date,
            applicable_count: day.applicable_count,
            completed_count: day.completed_count,
          }));
        }

        if (command === "set_task_checked") {
          const day = dayFixture[arguments_.localDate];
          const checkedTaskIds = new Set(day.checked_task_ids);

          if (arguments_.checked) {
            checkedTaskIds.add(arguments_.taskId);
          } else {
            checkedTaskIds.delete(arguments_.taskId);
          }

          day.checked_task_ids = [...checkedTaskIds];
          day.completed_count = day.checked_task_ids.length;
          return undefined;
        }

        throw new Error(`Unexpected command: ${command} for ${calendarDate}`);
      },
    };
  }, { dayFixture: fixture, calendarDate: yesterdayIso });
}

async function runDashboardAssertions() {
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1160, height: 820 } });

  await installTauriMock(page);
  await page.goto(serverUrl, { waitUntil: "networkidle" });

  await page.getByRole("heading", { name: "Today" }).waitFor();
  await page.getByLabel(`${yesterdayIso}, ${historicalCheckedIds.length} of ${taskIdsFor(yesterdayIso).length} complete`).click();
  await page.getByRole("heading", { name: "Selected day" }).waitFor();
  await page.getByText(`${historicalCheckedIds.length} / ${taskIdsFor(yesterdayIso).length} complete`).waitFor();

  const todayTask = page.getByLabel("Meals");
  await todayTask.uncheck();
  await page.getByText("3 / 6 complete").waitFor();
  const desktopScreenshot = resolve(repositoryRoot, ".agent-tasks/phase4-ui/dashboard-desktop.png");
  await page.screenshot({ path: desktopScreenshot, fullPage: true });

  await page.setViewportSize({ width: 390, height: 900 });
  await page.screenshot({ path: resolve(repositoryRoot, ".agent-tasks/phase4-ui/dashboard-narrow.png"), fullPage: true });
  console.log(`Dashboard screenshots written to ${desktopScreenshot}`);
  await browser.close();
}

const previewProcess = startPreviewServer();

try {
  await waitForServer();
  await runDashboardAssertions();
} finally {
  previewProcess.kill();
  await once(previewProcess, "exit");
}
