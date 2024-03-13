import { create } from "zustand";
import { Document } from "./utils";

interface StoreState {
    documents: Document[];
    selectedDoc: Document;
}

interface StoreActions {
    setDocuments: (newDocuments: Document[]) => void;
    setSelectedDoc: (newSelectedDoc: Document) => void;
}

export const useStore = create<StoreState & StoreActions>((set) => ({
    documents: [],
    selectedDoc: {} as Document,

    setDocuments: (newDocuments: Document[]) => set({ documents: newDocuments }),
    setSelectedDoc: (newSelectedDoc: Document) => set({ selectedDoc: newSelectedDoc })
}));
