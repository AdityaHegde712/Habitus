import { invoke } from "@tauri-apps/api/core";

export type TaskId =
  | "meals"
  | "sleep_7h"
  | "exercise"
  | "job_application"
  | "vitamins"
  | "leetcode_or_dsa"
  | "surfaces_clean";

export interface DailyRecordView {
  local_date: string;
  applicable_task_ids: TaskId[];
  checked_task_ids: TaskId[];
  applicable_count: number;
  completed_count: number;
  policy_version: number;
  updated_at_utc: string;
}

export interface CalendarDay {
  local_date: string;
  applicable_count: number;
  completed_count: number;
}

export const habitApi = {
  getDay(localDate: string): Promise<DailyRecordView> {
    return invoke<DailyRecordView>("get_day", { localDate });
  },

  listCalendarDays(): Promise<CalendarDay[]> {
    return invoke<CalendarDay[]>("list_calendar_days");
  },

  setTaskChecked(localDate: string, taskId: TaskId, checked: boolean): Promise<void> {
    return invoke<void>("set_task_checked", { localDate, taskId, checked });
  },
};
