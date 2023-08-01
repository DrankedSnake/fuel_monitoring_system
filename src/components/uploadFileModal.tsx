import { Show, createSignal } from "solid-js";
import "./modal.css";

export default function UploadFileModal(props){
    const [isOpen, setIsOpen] = createSignal(false);

    const upload = (event) => {
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
                    <form action="#">
                        {props.children}
                    </form>
                    <button onClick={upload}>Upload</button>
                    <button onClick={cancel}>Cancel</button>
                </div>
            </Show>
        </>
    )
};