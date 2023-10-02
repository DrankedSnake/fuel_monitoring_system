import { For } from "solid-js";
import "./searchForm.css"

type SearchField = {
    name: string,
    id: string,
    type: string,
    step: string,
    min: string,
    max: string,
    value: string,
}

type SearchFormProps = {
    fields: Array<SearchField>,
    submitCallback: Any,
    formChangeCallback: Any,
};

export default function SearchForm(props: SearchFormProps){
    return (
        <div class="search-box">
            <form action="submit">
                <For each={props.fields}>
                    {
                        (field) => (
                            <input 
                                class="search-box-item" 
                                type={field.type} 
                                name={field.name}
                                id={field.id}
                                step={field.step}
                                min={field.min}
                                max={field.max}
                                value={field.value}
                                placeholder={`Enter ${field.name}...`}
                                onchange={props.formChangeCallback(field.name)}
                            />
                        )
                    }
                </For>
            </form>
            <button class="submit" onclick={props.submitCallback}>Search</button>
        </div>
    )
}