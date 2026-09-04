import { fileURLToPath, pathToFileURL } from "node:url";
import { dirname, join } from "node:path";

import { chromium } from "playwright";

const prototypeDirectory = dirname(fileURLToPath(import.meta.url));
const prototypeUrl = pathToFileURL(join(prototypeDirectory, "prototype.html")).href;

async function capture(viewport, filename) {
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport });

  await page.goto(prototypeUrl, { waitUntil: "networkidle" });
  await page.getByLabel("September 14, 6 of 7 complete").click();

  const selectedDay = await page.locator("#selected-title").textContent();
  const selectedProgress = await page.locator("#selected-progress").textContent();
  if (selectedDay !== "Sep 14" || selectedProgress !== "6 / 7") {
    throw new Error(`Expected Sep 14 after calendar selection, received ${selectedDay}`);
  }

  await page.screenshot({
    path: join(prototypeDirectory, filename),
    fullPage: true,
  });
  await browser.close();
}

await capture({ width: 1160, height: 820 }, "prototype-desktop.png");
await capture({ width: 390, height: 900 }, "prototype-narrow.png");
