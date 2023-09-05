import { useWritable } from "./sharedStore";

interface State {
  initialized: boolean;
  loading: boolean;
  error: boolean;
  currentTile: number;
  breadcrumb: string
  currentFolderUuid: string
}

const state: State = {
  initialized: false,
  loading: false,
  error: false,
  currentTile: 0,
  breadcrumb: "",
  currentFolderUuid: ""
}

export const useStateStore = () => useWritable("state", state);

