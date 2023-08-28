import BarItem from "./barItem/barItem";
import NavigationItems from "../../data/navigationItems";
import { For, Show } from "solid-js";
import "./navigationBar.css"


export default function NavigationBar(){
    return (
        <Show when={NavigationItems()} fallback={<p>No items in menu</p>}>
            <section id="menu">
                <div class="icon">
                    <img src="src/icons/menu.png" alt="" />
                    <i>FMS</i>
                </div>
                <div class="items">
                    <For each={Object.values(NavigationItems())}>
                        {(item) => (
                            <BarItem 
                                title={item.item.title} 
                                path={item.item.path}
                                icon={item.item.icon}
                            >
                                Some
                            </BarItem>
                        )}
                    </For>
                </div>
            </section>
        </Show>
    )
}
