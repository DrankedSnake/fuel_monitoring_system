import { Show, createSignal } from "solid-js";
import { open } from "@tauri-apps/plugin-dialog";

export default function UploadFileModal(props) {
    const [isOpen, setIsOpen] = createSignal(false);
    const [filePath, setFilePath] = createSignal("");

    const openFileDialog = async () => {
        const selectedPath = await open({
            multiple: false,
        });

        if (selectedPath) {
            console.log("File path: ", selectedPath);
            setFilePath(selectedPath);
        }
    };

    const upload = async () => {
        setIsOpen(false);
        await props.submitFormCallback(filePath());
    };

    const cancel = () => {
        setIsOpen(false);
    };

    return (
        <>
            <div class="operations-bar">
                <button 
                    class="open-modal-button" 
                    onclick={() => { setIsOpen(true); }}
                >
                    {props.buttonText}
                </button>
            </div>
            <Show when={isOpen()}>
                <div class="modal flex-container-center">
                    <div class="modal-title">{props.title}</div>
                    <button onClick={openFileDialog}>Chose file</button>
                    <p>Chosen path to file: {filePath()}</p>
                    {props.children}
                    <div class="modal-control-bar">
                        <button class="submit" onClick={upload}>Upload</button>
                        <button class="cancel" onClick={cancel}>Cancel</button>
                    </div>
                </div>
            </Show>
        </>
    );
}