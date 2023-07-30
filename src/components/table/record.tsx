import { For } from "solid-js";
import "./record.css";


export default function Record(props){
    return (
        <tr class="grid-item">
            <For each={Object.values(props.value)}>
                {
                    (value)=>(
                        <td>{value}</td>
                    )
                }
            </For>
        </tr>
    )
}