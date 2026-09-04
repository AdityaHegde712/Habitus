import { habitApi, type CalendarDay, type DailyRecordView, type TaskId } from "./habit-api";
import "./styles.css";

const TASK_LABELS: Record<TaskId, string> = {
  meals: "Meals",
  sleep_7h: "7h sleep",
  exercise: "Exercise",
  job_application: "Job application",
  vitamins: "Vitamins",
  leetcode_or_dsa: "LeetCode / DSA",
  surfaces_clean: "Surfaces clean",
};

const CALENDAR_WEEK_COUNT = 18;
const DAYS_PER_WEEK = 7;
const dashboardRoot = document.querySelector<HTMLElement>("#app");

if (dashboardRoot === null) {
  throw new Error("The dashboard root is missing.");
}

const app: HTMLElement = dashboardRoot;

interface DashboardState {
  calendarDays: CalendarDay[];
  selectedDay: DailyRecordView;
  today: DailyRecordView;
}

function localDateIso(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");

  return `${year}-${month}-${day}`;
}

function dateBeforeToday(): string {
  const yesterday = new Date();
  yesterday.setDate(yesterday.getDate() - 1);

  return localDateIso(yesterday);
}

function displayDate(localDate: string, includeWeekday = false): string {
  const options: Intl.DateTimeFormatOptions = includeWeekday
    ? { weekday: "long", month: "long", day: "numeric" }
    : { month: "short", day: "numeric" };

  return new Intl.DateTimeFormat("en-US", options).format(new Date(`${localDate}T12:00:00`));
}

function taskMarkup(day: DailyRecordView, interactive: boolean): string {
  const checkedTaskIds = new Set(day.checked_task_ids);

  return day.applicable_task_ids
    .map((taskId) => {
      const isChecked = checkedTaskIds.has(taskId);
      const stateLabel = isChecked ? "done" : "open";
      const disabled = interactive ? "" : " disabled";

      return `<label class="task-row${isChecked ? " checked" : ""}">
        <input class="task-check" type="checkbox" data-task-id="${taskId}"${isChecked ? " checked" : ""}${disabled} />
        <span class="task-label">${TASK_LABELS[taskId]}</span>
        <span class="task-meta">${stateLabel}</span>
      </label>`;
    })
    .join("");
}

function historicalTaskMarkup(day: DailyRecordView): string {
  const checkedTaskIds = new Set(day.checked_task_ids);

  return day.applicable_task_ids
    .map((taskId) => {
      const isChecked = checkedTaskIds.has(taskId);
      const stateLabel = isChecked ? "complete" : "open";
      const icon = isChecked ? "✓" : "○";

      return `<div class="detail-row">
        <span class="detail-icon${isChecked ? "" : " open"}">${icon}</span>
        <span>${TASK_LABELS[taskId]}</span>
        <span class="detail-state">${stateLabel}</span>
      </div>`;
    })
    .join("");
}

function calendarStartDate(today: Date): Date {
  const start = new Date(today);
  start.setDate(start.getDate() - (CALENDAR_WEEK_COUNT * DAYS_PER_WEEK - 1));

  return start;
}

function calendarMarkup(calendarDays: CalendarDay[], selectedDate: string): string {
  const recordsByDate = new Map(calendarDays.map((day) => [day.local_date, day]));
  const today = new Date();
  const start = calendarStartDate(today);
  const cells: string[] = [];

  for (let offset = 0; offset < CALENDAR_WEEK_COUNT * DAYS_PER_WEEK; offset += 1) {
    const date = new Date(start);
    date.setDate(start.getDate() + offset);
    const localDate = localDateIso(date);
    const record = recordsByDate.get(localDate);
    const completionRatio = record === undefined ? 0 : record.completed_count / record.applicable_count;
    const completionLevel = Math.ceil(completionRatio * 4);
    const label = record === undefined
      ? `${displayDate(localDate)}, no saved checklist`
      : `${displayDate(localDate)}, ${record.completed_count} of ${record.applicable_count} complete`;

    cells.push(`<button class="day" type="button" data-date="${localDate}" data-level="${completionLevel}" aria-label="${label}" aria-pressed="${localDate === selectedDate}"></button>`);
  }

  return cells.join("");
}

function renderDashboard(state: DashboardState): void {
  app.innerHTML = `<main class="app-shell">
    <header class="topbar">
      <div><h1>Habit Tracker</h1><p class="eyebrow">${displayDate(state.today.local_date, true)}</p></div>
      <span class="status-chip">Local only</span>
    </header>
    <section class="card today" aria-labelledby="today-title">
      <div class="section-heading"><div><h2 id="today-title">Today</h2><p class="section-copy">Keep the basics moving. Changes save immediately.</p></div><span class="progress">${state.today.completed_count} / ${state.today.applicable_count} complete</span></div>
      <div class="task-grid">${taskMarkup(state.today, true)}</div>
    </section>
    <section class="history" aria-label="Habit history">
      <article class="card tracker"><div class="section-heading"><div><h2>Consistency</h2><p class="section-copy">Select a day to inspect its saved checklist.</p></div></div><div class="months" role="group" aria-label="Completion tracker">${calendarMarkup(state.calendarDays, state.selectedDay.local_date)}</div><div class="legend" aria-label="Completion intensity legend"><span>Less</span><span class="legend-square"></span><span class="legend-square"></span><span class="legend-square"></span><span class="legend-square"></span><span class="legend-square"></span><span>More</span></div></article>
      <aside class="card selected-day" aria-live="polite" aria-labelledby="selected-title"><div class="selected-heading"><h2 id="selected-title">Selected day · ${displayDate(state.selectedDay.local_date)}</h2><span class="progress">${state.selectedDay.completed_count} / ${state.selectedDay.applicable_count} complete</span></div><p class="detail-copy">Historical checklist · saved snapshot</p><div class="detail-list">${historicalTaskMarkup(state.selectedDay)}</div></aside>
    </section>
    <p class="error-message" id="error-message" role="alert"></p>
  </main>`;
}

function renderError(error: unknown): void {
  const message = error instanceof Error ? error.message : String(error);
  const errorMessage = document.querySelector<HTMLElement>("#error-message");

  if (errorMessage !== null) {
    errorMessage.textContent = `Could not update the dashboard: ${message}`;
    return;
  }

  app.innerHTML = `<main class="app-shell"><section class="card startup-error" aria-labelledby="startup-error-title"><h1 id="startup-error-title">Habit Tracker could not load</h1><p class="error-message" role="alert"></p></section></main>`;
  const startupError = document.querySelector<HTMLElement>(".startup-error .error-message");

  if (startupError !== null) {
    startupError.textContent = `Could not load local habit data: ${message}`;
  }
}

async function loadDashboard(selectedDate = dateBeforeToday()): Promise<void> {
  try {
    const todayDate = localDateIso(new Date());
    const [today, selectedDay, calendarDays] = await Promise.all([habitApi.getDay(todayDate), habitApi.getDay(selectedDate), habitApi.listCalendarDays()]);
    renderDashboard({ today, selectedDay, calendarDays });
  } catch (error) {
    renderError(error);
  }
}

async function handleTaskChange(input: HTMLInputElement): Promise<void> {
  const taskId = input.dataset.taskId as TaskId | undefined;
  if (taskId === undefined) return;

  input.disabled = true;
  try {
    await habitApi.setTaskChecked(localDateIso(new Date()), taskId, input.checked);
    await loadDashboard();
  } catch (error) {
    input.checked = !input.checked;
    input.disabled = false;
    renderError(error);
  }
}

app.addEventListener("change", (event) => {
  const target = event.target;
  if (target instanceof HTMLInputElement && target.matches(".task-check")) void handleTaskChange(target);
});

app.addEventListener("click", (event) => {
  const target = event.target;
  if (!(target instanceof HTMLButtonElement) || !target.matches(".day")) return;

  const selectedDate = target.dataset.date;
  if (selectedDate !== undefined) void loadDashboard(selectedDate);
});

void loadDashboard();
