import { produce } from "immer";
import { create } from "zustand";
import { ApiError, fetchOntologyStatementJson } from "../../../infrastructure/api/api-client";
import type { OntologyEntity } from "./ontology-types";
import { transformOntologyStatementJson } from "./transform-ontology-statement-json";

interface OntologyViewerState {
  entities: OntologyEntity[];
  errorMessage: string | null;
  filterText: string;
  hasLoaded: boolean;
  isLoading: boolean;
  loadOntology: () => Promise<void>;
  reset: () => void;
  selectedEntityId: string | null;
  selectEntity: (entityId: string) => void;
  setFilterText: (value: string) => void;
}

const EMPTY_STATE = {
  entities: [],
  selectedEntityId: null,
  errorMessage: null,
  hasLoaded: false,
  isLoading: true,
  filterText: "",
} satisfies Pick<
  OntologyViewerState,
  "entities" | "selectedEntityId" | "errorMessage" | "hasLoaded" | "isLoading" | "filterText"
>;

let ontologyLoadPromise: Promise<void> | null = null;

export const useOntologyViewerStore = create<OntologyViewerState>((set, get) => ({
  ...EMPTY_STATE,
  loadOntology: async () => {
    const currentState = get();

    if (currentState.hasLoaded) {
      return;
    }

    if (ontologyLoadPromise) {
      return ontologyLoadPromise;
    }

    set((current) =>
      produce(current, (draft) => {
        draft.isLoading = true;
        draft.errorMessage = null;
      }),
    );

    ontologyLoadPromise = (async () => {
      try {
        const ontologyStatementJson = await fetchOntologyStatementJson();
        const entities = transformOntologyStatementJson(ontologyStatementJson);

        set((current) =>
          produce(current, (draft) => {
            draft.entities = entities;
            draft.errorMessage = null;
            draft.hasLoaded = true;
            draft.isLoading = false;

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
        );
      } catch (error) {
        const errorMessage =
          error instanceof ApiError
            ? error.message
            : error instanceof Error
              ? error.message
              : "The ontology viewer failed for an unknown reason.";

        set((current) =>
          produce(current, (draft) => {
            draft.entities = [];
            draft.errorMessage = errorMessage;
            draft.isLoading = false;
          }),
        );
      } finally {
        ontologyLoadPromise = null;
      }
    })();

    return ontologyLoadPromise;
  },
  selectEntity: (entityId) =>
    set((current) =>
      produce(current, (draft) => {
        draft.selectedEntityId = entityId;
      }),
    ),
  setFilterText: (value) =>
    set((current) =>
      produce(current, (draft) => {
        draft.filterText = value;
      }),
    ),
  reset: () =>
    (() => {
      ontologyLoadPromise = null;

      set((current) =>
        produce(current, (draft) => {
          draft.entities = EMPTY_STATE.entities;
          draft.selectedEntityId = EMPTY_STATE.selectedEntityId;
          draft.errorMessage = EMPTY_STATE.errorMessage;
          draft.hasLoaded = EMPTY_STATE.hasLoaded;
          draft.isLoading = EMPTY_STATE.isLoading;
          draft.filterText = EMPTY_STATE.filterText;
        }),
      );
    })(),
}));
