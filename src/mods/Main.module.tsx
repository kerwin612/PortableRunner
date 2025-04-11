import { forwardRef, useEffect, useRef, useState } from 'react';
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { getCurrentWindow } from "@tauri-apps/api/window";

import CmdItem from './coms/CmdItem.component';
import { runCmd, readLnk, addShortcut, cfgEpoch, loadCmds } from '../libs/apis';
import { getCmdlineWithFile, isDirPath, isValidPath, getFileNameWithoutExt } from '../libs/utils';
import styles from './Main.module.css';



//To be compatible, the latest value of currentCmds cannot be obtained in runWithFile
const GLOBAL_CMDS: any[] = [];

const CmdInput = forwardRef<HTMLInputElement, { cmdStr?: string, onChange: (e: any) => void, onEnter: (value: string) => void }>((props, ref) => {
    const [inputValue, setInputValue] = useState<string>('');

    const onInputKeydown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.keyCode !== 13) return;
        props.onEnter(inputValue);
        setInputValue('');
    };

    useEffect(() => {
        setInputValue(props.cmdStr || '');
    }, [props.cmdStr]);

    return (
        <input ref={ref} type="search" placeholder="cmd..." value={inputValue} onChange={props.onChange} onKeyDown={onInputKeydown} />
    );
});

const CmdList = ({ cmds, onClick }: {
    cmds: any[],
    onClick: (target: any, item: any) => void
}) => {
    const groups = new Map<string, any[]>();
    cmds.forEach((i: any) => {
        let group = i.group ?? "default";
        let value = groups.get(group) || [];
        value[value.length] = i;
        groups.set(group, value);
    });
    return (
        <>
            {
                Array.from(groups.keys()).map((key: string) => {
                    return (
                        <div key={key} className="cmd_sub_list">
                            {
                                groups.get(key)?.map((i: any, index: number) => {
                                    return (
                                        <CmdItem
                                            key={`${index}`}
                                            item={i}
                                            getCMD={i => i.run}
                                            onClick={onClick}
                                        />
                                    );
                                })
                            }
                        </div>
                    );
                })
            }
        </>
    );
};

export default function Main({ cmdStr, onWithFile }: {
    cmdStr?: string;
    onWithFile: (file: string, matchs: any[]) => void;
}) {

    const epoch = useRef<number>(0);
    const inputElement = useRef<any>(null);
    const unListenFileDrop = useRef<any>(null);
    const unListenFocusChanged = useRef<any>(null);
    const [inputValue, setInputValue] = useState<string>('');
    const [currentCmds, setCurrentCmds] = useState<any[]>([]);

    const refreshCmd = () => {
        cfgEpoch().then((current: number) => {
            if (epoch.current === current) return;
            loadCmds().then(list => setCurrentCmds(list));
            epoch.current = current;
        });
    };

    const setInputValueAndFocus = (cmdValue: string) => {
        setInputValue(cmdValue);
        if (inputElement.current) {
            inputElement.current.focus();
        }
    }

    const runWithFile = async (file: string) => {
        let isDir = await isDirPath(file);
        let matchs: any[] = [];
        GLOBAL_CMDS.forEach(i => {
            if (!!!(i.withFile)) return;
            i.withFile.forEach((j: any) => {
                if (j.pattern && new RegExp(j.pattern).test(file)) {
                    if (((j?.folderRequired ?? false) === true && !isDir) || ((j?.fileRequired ?? false) === true && isDir)) {
                        return;
                    }
                    matchs[matchs.length] = { ...i, cmd: `${i.run} ${j.parameters}`, withFile: j };
                }
            });
        });
        if (matchs.length === 0) {
            setInputValueAndFocus(`"${file}"`);
        } else if (matchs.length === 1) {
            let i = matchs[0];
            let cmdValue = getCmdlineWithFile(i, file, i.withFile);
            if (i.withFile.argumentsRequired) {
                setInputValueAndFocus(`${cmdValue} `);
            } else {
                await runCmd(cmdValue);
            }
        } else {
            onWithFile(file, matchs);
        }
    };

    const onInputEnter = async (cmdStr: string) => {
        if (isValidPath(cmdStr)) {
            await runWithFile(cmdStr);
        } else {
            await runCmd(cmdStr);
        }
    };

    const onFileDrop = async (file: string) => {
        if (file.endsWith('.lnk')) {
            readLnk(file).then(async (lnkInfo: any) => {
                addShortcut({
                    cmd: (lnkInfo?.name ?? '') || await getFileNameWithoutExt(lnkInfo.target),
                    run: `"${lnkInfo.target}" ${lnkInfo?.arguments ?? ''}`,
                }).then(() => {
                    refreshCmd();
                });
            });
        } else {
            await runWithFile(file);
        }
    };

    useEffect(() => {
        GLOBAL_CMDS.length = 0;
        GLOBAL_CMDS.push(...currentCmds);
    }, [currentCmds]);

    useEffect(() => {
        setInputValue(cmdStr || '');
    }, [cmdStr]);

    useEffect(() => {

        //onFocus
        unListenFocusChanged.current = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
            if (focused) {
                refreshCmd();
            }
        });

        //onFileDrop
        unListenFileDrop.current = getCurrentWebview().onDragDropEvent(async (event: any) => {
            if (event?.payload?.type !== 'drop' || (event?.payload?.paths || []).length < 0) return;
            await onFileDrop(event.payload.paths[0]);
        });

        return () => {
            if (unListenFocusChanged.current) {
                (async () => {
                    (await unListenFocusChanged.current)();
                })();
            }
            if (unListenFileDrop.current) {
                (async () => {
                    (await unListenFileDrop.current)();
                })();
            }
        };

    }, []);

    refreshCmd();

    return (
        <div className={styles.container}>
            <div className="cmd_main">
                <CmdInput
                    ref={inputElement}
                    cmdStr={inputValue}
                    onEnter={onInputEnter}
                    onChange={e => setInputValue(e.target.value)}
                />
            </div>
            <div className="cmd_list">
                <CmdList
                    cmds={currentCmds}
                    onClick={async (t, i) => {
                        if (t.classList.contains("disabled")) return;
                        t.classList.add("disabled");
                        if (i.argumentsRequired) {
                            setInputValueAndFocus(`${i.run} `);
                        } else if (i.type === 'file') {
                            await runWithFile(i.run);
                        } else {
                            await runCmd(i.run);
                        }
                        setTimeout(() => {
                            t.classList.remove("disabled");
                        }, 80);
                    }}
                />
            </div>
        </div>
    );
}
