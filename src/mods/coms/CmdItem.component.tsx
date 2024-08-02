import { useEffect, useRef } from "react";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";



export default function CmdItem({ item, getCMD, onClick }: {
    item: any;
    getCMD: (i: any) => string;
    onClick: (t: any, i: any) => void;
}) {

    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {

        let { current } = ref;
        if (current) {
            let target = current;
            target.setAttribute('style', item.style);
            target.onclick = () => {
                onClick(target, item);
            };
            target.onmousedown = (e) => {
                let isRightMB;
                e = e || window.event;

                if ("which" in e)  // Gecko (Firefox), WebKit (Safari/Chrome) & Opera
                    isRightMB = (e as MouseEvent).which == 3;
                else if ("button" in e)  // IE, Opera
                    isRightMB = (e as MouseEvent).button == 2;

                if (isRightMB) {
                    writeText(getCMD(item)).then(() => {
                        //
                    });
                }
            };
        }

    }, []);

    return (
        <div ref={ref} className="cmd_item">
            <span>{item.label ? (item.label + "(" + item.cmd + ")") : item.cmd}</span>
        </div>
    );

}
