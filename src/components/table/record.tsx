import { For } from "solid-js";
import "./record.css";

type RecordProps = {
    headers: Array<string>,
    record: Object,
}

export default function Record(props: RecordProps){
    return (
        <tr class="row">
            <For each={props.headers}>
                {
                    (header)=>{
                        return <td>{props.record[header]}</td>
                    }
                }
            </For>
        </tr>

    )
}