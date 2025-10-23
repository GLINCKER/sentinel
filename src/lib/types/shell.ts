export type ShellId = string;

export type ShellType = 'Bash' | 'Zsh' | 'Fish' | 'PowerShell' | 'Cmd' | 'Sh';

export type ShellStatus = 'Active' | 'Inactive' | 'Error';

export interface ShellInfo {
  id: ShellId;
  pid: number | null;
  process_name: string | null;
  cwd: string;
  shell_type: ShellType;
  created_at: string;
  status: ShellStatus;
}

export interface ShellOutput {
  shell_id: ShellId;
  data: string;
}

