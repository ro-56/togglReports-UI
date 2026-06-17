
export type OptionItem = {
  label: string;
  command: string;
  requires_date_range: boolean;
}

export type LastUsedOptions = {
  command?: string;
  start_date?: string;
  end_date?: string;
}

export type ReportVersion = 'v1' | 'v2';
export type EffortType = 'HORAS' | 'MINUTOS' | 'HHMM';
export type ReportEncoding = 'UTF8' | 'WINDOWS1252';

export type Config = {
  api_token: string;
  workspace_filter?: number;
  output_file_dir: string;
  sgu_name: string;
  ignore_tag?: string;
  default_tag?: string;
  report_version: ReportVersion;
  effort_type: EffortType;
  report_encoding: ReportEncoding;
}

export type Workspace = {
  id: number;
  name: string;
}