import "./barItem.css"


export default function BarItem(props: any){
    return (
        <>
            <a class="item" href={props.path}>
                <li>
                    <img class="icon" src={props.icon}/>
                    <span class="item-label">
                        <i>
                            {props.title}
                        </i>
                    </span>
                </li>
            </a>
        </>
    )
}
