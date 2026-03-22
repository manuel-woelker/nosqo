import { produce } from "immer";
import { create } from "zustand";

interface UiShellState {
  closeNavigation: () => void;
  isNavigationOpen: boolean;
  openNavigation: () => void;
  toggleNavigation: () => void;
}

export const useUiShellStore = create<UiShellState>((set) => ({
  isNavigationOpen: false,
  openNavigation: () =>
    set((current) =>
      produce(current, (draft) => {
        draft.isNavigationOpen = true;
      }),
    ),
  closeNavigation: () =>
    set((current) =>
      produce(current, (draft) => {
        draft.isNavigationOpen = false;
      }),
    ),
  toggleNavigation: () =>
    set((current) =>
      produce(current, (draft) => {
        draft.isNavigationOpen = !draft.isNavigationOpen;
      }),
    ),
}));
