import { For } from "solid-js";
import "./record.css";


export default function Record(props){
    return (
        <tr class="grid-item">
            <For each={props.headers}>
                {
                    (header)=>(
                        <td>{props.record[header]}</td>
                    )
                }
            </For>
        </tr>
    )
}