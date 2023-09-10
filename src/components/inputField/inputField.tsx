import "./inputField.css";


type InputFieldType = {
    placeholder: string,
    type: string,
    name: string,
    id: string,
    onChange: any,
    required: string,
    labelText: string,
    min: string,
    max: string,
    step: string,
};

export default function InputField(props: InputFieldType){
    return (
        <div class="inputBox">
            <input 
                placeholder={props.placeholder} 
                type={props.type}
                name={props.name} 
                id={props.id} 
                onChange={props.onChange}
                step={props.step}
                min={props.min}
                max={props.max}
            />
            <span>{props.labelText}</span>
        </div>
    )
}