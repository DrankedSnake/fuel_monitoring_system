import "./bar_item.css"


export default function BarItem(props: any){
    return (
        <>
            <a class="navigation-item" href={props.path}>{props.title}</a>
            <div></div>
        </>
    )
}
