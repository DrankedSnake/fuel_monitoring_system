import BarItem from "./bar_item/bar_item";
import items from "../../data/navigation_items";
import { For, Show } from "solid-js";


export default function NavigationBar(){
    return (
        <Show when={items()} fallback={<p>Loading...</p>}>
            <div id="navigation-bar">
                <For each={items()}>
                    {(item) => (
                        <BarItem title={item.title} path={item.path}>Some</BarItem>
                    )}
                </For>
            </div>
        </Show>
    )
}
