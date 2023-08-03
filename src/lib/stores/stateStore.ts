import { useWritable } from "./sharedStore";

interface State {
  initialized: boolean;
  loading: boolean;
  error: boolean;
}

const state: State = {
  initialized: false,
  loading: false,
  error: false,

}

export const useStateStore = () => useWritable("state", state);

