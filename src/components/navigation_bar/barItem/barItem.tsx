import "./barItem.css"


export default function BarItem(props: any){
    return (
        <>
            <a class="item" href={props.path}>
                <li>
                    <img class="icon" src={props.icon}/>
                    <i class={props.title}>
                        {props.title}
                    </i>
                    
                </li>
            </a>
        </>
    )
}
