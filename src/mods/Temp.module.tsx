import CmdItem from './coms/CmdItem.component';
import { runCmd } from '../libs/apis';
import { getCmdlineWithFile } from '../libs/utils';
import styles from './Temp.module.css';



export default function Temp({ file, matchs, onBackClick, onArgumentsRequired }: {
    file: string;
    matchs: any[];
    onBackClick: () => void;
    onArgumentsRequired: (cmd: string) => void;
}) {
    return (
        <div className={styles.container}>
            <div className="cmd_main">
                <input type="search" value={file} readOnly />
                <button onClick={onBackClick}>Back</button>
            </div>
            <div className="cmd_list">
                {
                    matchs.map((i: any, index: number) => {
                        let cmd = getCmdlineWithFile(i, file, i.withFile);
                        return (
                            <CmdItem
                                key={`${index}`}
                                item={i}
                                getCMD={() => cmd}
                                onClick={async (t, i) => {
                                    if (t.classList.contains("disabled")) return;
                                    t.classList.add("disabled");
                                    if (i.withFile.argumentsRequired) {
                                        await onArgumentsRequired(cmd);
                                    } else {
                                        await runCmd(cmd);
                                    }
                                    setTimeout(() => {
                                        t.classList.remove("disabled");
                                    }, 80);
                                }}
                            />
                        );
                    })
                }
            </div>
        </div>
    );
}
