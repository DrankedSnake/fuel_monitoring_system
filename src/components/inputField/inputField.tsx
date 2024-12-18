import { useForm } from "../validation";
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
    validator: Function,
};


const ErrorMessage = (props) => {
    return (
        <div class="error-message">{props.error}</div>
    )
};


export default function InputField(props: InputFieldType){
    const { validate, formSubmit, errors } = useForm({
        errorClass: "error-input"
    });
    if (!props.validator){
        props.validator = () => {return ""}
    }
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
                required={props.required}
                use:validate={[props.validator]}
            />
            <span>{props.labelText}</span>
            {errors[props.name] ? <ErrorMessage error={errors[props.name]}/> : <span></span> }
        </div>
    )
}