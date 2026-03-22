import { produce } from "immer";
import { create } from "zustand";
import type { OntologyEntity } from "./ontology-types";

interface OntologyViewerState {
  entities: OntologyEntity[];
  errorMessage: string | null;
  filterText: string;
  isLoading: boolean;
  reset: () => void;
  selectedEntityId: string | null;
  selectEntity: (entityId: string) => void;
  setEntities: (entities: OntologyEntity[]) => void;
  setErrorMessage: (message: string | null) => void;
  setFilterText: (value: string) => void;
  setIsLoading: (value: boolean) => void;
}

const EMPTY_STATE = {
  entities: [],
  selectedEntityId: null,
  errorMessage: null,
  isLoading: true,
  filterText: "",
} satisfies Pick<
  OntologyViewerState,
  "entities" | "selectedEntityId" | "errorMessage" | "isLoading" | "filterText"
>;

export const useOntologyViewerStore = create<OntologyViewerState>((set) => ({
  ...EMPTY_STATE,
  setEntities: (entities) =>
    set((current) =>
      produce(current, (draft) => {
        draft.entities = entities;

        if (entities.length === 0) {
          draft.selectedEntityId = null;
          return;
        }

        const hasExistingSelection = entities.some(
          (entity) => entity.id === current.selectedEntityId,
        );

        if (!hasExistingSelection) {
          draft.selectedEntityId = entities[0]?.id ?? null;
        }
      }),
    ),
  selectEntity: (entityId) =>
    set((current) =>
      produce(current, (draft) => {
        draft.selectedEntityId = entityId;
      }),
    ),
  setErrorMessage: (message) =>
    set((current) =>
      produce(current, (draft) => {
        draft.errorMessage = message;
      }),
    ),
  setIsLoading: (value) =>
    set((current) =>
      produce(current, (draft) => {
        draft.isLoading = value;
      }),
    ),
  setFilterText: (value) =>
    set((current) =>
      produce(current, (draft) => {
        draft.filterText = value;
      }),
    ),
  reset: () =>
    set((current) =>
      produce(current, (draft) => {
        draft.entities = EMPTY_STATE.entities;
        draft.selectedEntityId = EMPTY_STATE.selectedEntityId;
        draft.errorMessage = EMPTY_STATE.errorMessage;
        draft.isLoading = EMPTY_STATE.isLoading;
        draft.filterText = EMPTY_STATE.filterText;
      }),
    ),
}));
