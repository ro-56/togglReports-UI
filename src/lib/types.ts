
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

export type Config = {
  api_token: string;
  output_file_dir: string;
  sgu_name: string;
  ignore_tag: string;
  default_tag: string;
}

export type Workspace = {
  id: number;
  name: string;
}