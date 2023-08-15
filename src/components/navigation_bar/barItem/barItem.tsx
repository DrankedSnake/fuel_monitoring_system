import "./barItem.css"


export default function BarItem(props: any){
    return (
        <>
            <li><i class={props.title}></i><a class="navigation-item" href={props.path}>{props.title}</a></li>
        </>
    )
}
