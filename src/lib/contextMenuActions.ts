import type { FileNode } from "$lib/bindings";
import type { ContextMenuItem } from "$lib/types";
import { openModal, closeModal } from "$lib/modalStore";
import { renamePath, deletePath, createFile, createFolder } from "$lib/actions";

// Import modal components that can be triggered from the context menu
import TextInputModal from "./components/TextInputModal.svelte";
import ConfirmModal from "./components/ConfirmModal.svelte";

/**
 * This function dynamically builds the list of actions for the context menu
 * based on the node that was clicked (file vs. folder).
 *
 * @param node The FileNode that was right-clicked.
 * @returns An array of ContextMenuItem objects for the menu.
 */
export function getContextMenuActions(node: FileNode): ContextMenuItem[] {
    const isDir = !!node.children;
    let actions: ContextMenuItem[] = [
        {
            label: "Rename",
            handler: () => {
                openModal({
                    component: TextInputModal,
                    props: {
                        title: `Rename ${isDir ? "Folder" : "File"}`,
                        label: `New name for '${node.name}'`,
                        initialValue: node.name,
                        buttonText: "Rename",
                        onClose: closeModal,
                        onSubmit: (newValue: string) => {
                            renamePath(node.path, newValue);
                            closeModal();
                        },
                    },
                });
            },
        },
        {
            label: "Delete",
            handler: () => {
                openModal({
                    component: ConfirmModal,
                    props: {
                        title: `Delete ${isDir ? "Folder" : "File"}`,
                        message: `Are you sure you want to delete '${node.name}'? This action cannot be undone.`,
                        onClose: closeModal,
                        onConfirm: () => {
                            deletePath(node.path);
                            closeModal();
                        },
                    },
                });
            },
        },
    ];

    if (isDir) {
        actions.push({ isSeparator: true });
        actions.push({
            label: "New File...",
            handler: () => {
                openModal({
                    component: TextInputModal,
                    props: {
                        title: "New File",
                        label: "Enter the name for the new file:",
                        buttonText: "Create",
                        onClose: closeModal,
                        onSubmit: (name: string) => {
                            createFile(node.path, name);
                            closeModal();
                        },
                    },
                });
            },
        });
        actions.push({
            label: "New Folder...",
            handler: () => {
                openModal({
                    component: TextInputModal,
                    props: {
                        title: "New Folder",
                        label: "Enter the name for the new folder:",
                        buttonText: "Create",
                        onClose: closeModal,
                        onSubmit: (name: string) => {
                            createFolder(node.path, name);
                            closeModal();
                        },
                    },
                });
            },
        });
    }

    return actions;
}
