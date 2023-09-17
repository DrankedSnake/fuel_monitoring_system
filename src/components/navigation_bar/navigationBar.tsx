import BarItem from "./barItem/barItem";
import NavigationItems from "../../data/navigationItems";
import { For, Show, createSignal } from "solid-js";
import "./navigationBar.css"


export default function NavigationBar(){
    const [navigationClass, setNavigationClass] = createSignal("navigation-container navigation-container--collapsed");

    const switchMenu = () => {
        if (navigationClass() === "navigation-container navigation-container--collapsed") {
            setNavigationClass("navigation-container")
        } else {
            setNavigationClass("navigation-container navigation-container--collapsed")
        }
    };

    return (
        <Show when={NavigationItems()} fallback={<p>No items in menu</p>}>
            <div class={navigationClass()}>
                <div class="application-menu" onclick={switchMenu}>
                    <img src="icons/menu.png" alt="" />
                    <span class="item-label">
                        <i>FMS</i>
                    </span>
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
            </div>
        </Show>
    )
}
