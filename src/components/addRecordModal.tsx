import { Show, createSignal } from "solid-js";
import "./modal.css";


export default function AddRecordModal(props){
    const [isOpen, setIsOpen] = createSignal(false)

    const addRecord = (event) => {
        setIsOpen(false);
    };

    const cancel = (event) => {
        setIsOpen(false);
    };

    return (
        <>
            <div class="operations-bar">
                <button onclick={()=>{setIsOpen(true)}}>{props.buttonText}</button>
            </div>
            <Show when={isOpen()}>
                <div class="modal">
                    <div>{props.title}</div>
                    <form action="submit">
                        {props.children}
                    </form>
                    <button onClick={addRecord}>Add</button>
                    <button onClick={cancel}>Cancel</button>
                </div>
            </Show>
        </>
    )
};