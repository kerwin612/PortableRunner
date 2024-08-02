import { useEffect, useState } from 'react';

import Init from './mods/Init.module';
import Temp from './mods/Temp.module';
import Main from './mods/Main.module';

import { loadSet, saveSet } from './libs/apis';
import './App.css';



export default function App() {

    const [page, setPage] = useState('');
    const [cmdStr, setCmdStr] = useState('');
    const [file, setFile] = useState('');
    const [matchs, setMatchs] = useState<any[]>([]);

    useEffect(() => {

        //onLoaded
        loadSet().then(async (s: any) => {
            if (!(s.tpath) || !(s.lpath)) {
                setPage('init');
            } else {
                const result = await saveSet({
                    tpath: s.tpath,
                    lpath: s.lpath,
                    hpath: s.hpath.trim() || ".home"
                });
                if (result) {
                    setPage('main');
                } else {
                    setPage('init');
                }
            }
        });

        //onContextMenu
        const handleContextMenu = (event: MouseEvent) => {
            event.preventDefault();
        };

        document.addEventListener('contextmenu', handleContextMenu, false);

        return () => {
            document.removeEventListener('contextmenu', handleContextMenu, false);
        };

    }, []);

    return (
        <div className='container'>
            {page === 'init' &&
                <Init
                    onSaveSuccess={() => { setPage('main'); }}
                />
            }
            {page === 'temp' &&
                <Temp
                    file={file}
                    matchs={matchs}
                    onBackClick={() => { setPage('main'); }}
                    onArgumentsRequired={(cmd) => {
                        setCmdStr(cmd);
                        setPage('main');
                    }}
                />
            }
            {page === 'main' &&
                <Main
                    cmdStr={cmdStr}
                    onWithFile={(file: string, matchs: any[]) => {
                        setFile(file);
                        setMatchs(matchs);
                        setPage('temp');
                    }}
                />
            }
        </div>
    );
}
