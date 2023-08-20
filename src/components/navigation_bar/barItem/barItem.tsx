import "./barItem.css"


export default function BarItem(props: any){
    return (
        <>
            <a class="navigation-item" href={props.path}><li><i class={props.title}></i>{props.title}</li></a>
        </>
    )
}
