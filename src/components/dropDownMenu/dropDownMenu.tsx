import { For, Show } from "solid-js";
import "./dropDownMenu.css"


type DropDownMenuProps = {
    items: Array<Object>,
    displayValueKey: string,
    identifyValueKey: string,
    setSignalCallback: Function,
    placeholder: string,
    setStorageCallback?: Function,
};


export default function DropDownMenu(props: DropDownMenuProps){

    const triggerDropDown = (event: Event) => {
        let element = event.currentTarget as HTMLDivElement;
        element.classList.toggle("active")
    };

    const inputElement = () => {
        return <input type="text" placeholder={props.placeholder} class="textBox" readOnly/>
    };
    const inputField = inputElement();

    return (
        <div class="select" onClick={triggerDropDown}>    
            {inputField}
            <div class="option">
                <Show when={props.items}>
                    <For each={props.items}>
                    {
                        (item: Object) => (
                            <div 
                                onClick={
                                    ()=>{
                                        inputField.value = item[props.displayValueKey];

                                        if (props.setSignalCallback) {
                                            props.setSignalCallback(item[props.identifyValueKey]);
                                        }
                                        if (props.setStorageCallback) {
                                            props.setStorageCallback(item);
                                        }
                                    }
                                }
                                id={item[props.identifyValueKey]}>{item[props.displayValueKey]}
                            </div>
                        )
                    }
                    </For>
                </Show>
            </div>
        </div>
    )
}