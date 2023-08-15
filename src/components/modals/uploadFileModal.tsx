import { Show, createSignal } from "solid-js";

export default function UploadFileModal(props){
    const [isOpen, setIsOpen] = createSignal(false);

    const upload = async (event: Event) => {
        props.submitFormCallback();
        setIsOpen(false);
    };

    const cancel = (event) => {
        setIsOpen(false);
    };

    return (
        <>
            <div class="operations-bar">
                <button 
                    class="open-modal-button" 
                    onclick={()=>{setIsOpen(true)}}
                >
                    {props.buttonText}
                </button>
            </div>
            <Show when={isOpen()}>
                <div class="modal flex-container-center">
                    <div class="modal-title">{props.title}</div>
                    <form action="#" method="post" enctype="multipart/form-data">
                        {props.children}
                    </form>
                    <div class="modal-control-bar">
                        <button class="submit" onClick={upload}>Upload</button>
                        <button class="cancel" onClick={cancel}>Cancel</button>
                    </div>
                    
                </div>
            </Show>
        </>
    )
};