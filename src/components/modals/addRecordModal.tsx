import { Show, createSignal } from "solid-js";
import "./modal.css"

export default function AddRecordModal(props){
    const [isOpen, setIsOpen] = createSignal(false)

    const addRecord = async (event) => {
        await props.add_record_callback();
        setIsOpen(false);
    };

    const cancel = (event) => {
        setIsOpen(false);
    };

    return (
        <>
            <div class="operations-bar">
                <button class="open-modal-button" onclick={()=>{setIsOpen(true)}}>{props.buttonText}</button>
            </div>
            <Show when={isOpen()}>
                <div class="modal flex-container-center">
                    <div class="modal-title">{props.title}</div>
                    <form action="submit">
                        {props.children}
                    </form>
                    <div class="modal-control-bar">
                        <button class="submit" onClick={addRecord}>Add</button>
                        <button class="cancel" onClick={cancel}>Cancel</button>
                    </div>
                </div>
            </Show>
        </>
    )
};