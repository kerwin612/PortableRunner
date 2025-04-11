
import React, { useState } from 'react';

import { saveSet } from '../libs/apis.js';
import styles from './Init.module.css';



export default function Init({ onSaveSuccess }: {
    onSaveSuccess: () => void;
}) {
    const [inputs, setInputs] = useState({
        tpath: "",
        lpath: "",
        hpath: ".home"
    });

    const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = event.target;
        setInputs(prevInputs => ({ ...prevInputs, [name]: value }));
    };

    const save = async () => {
        const result = await saveSet({
            tpath: inputs.tpath,
            lpath: inputs.lpath,
            hpath: inputs.hpath.trim() || ".home"
        });
        return result;
    }

    const doSave = async () => {
        if (await save()) {
            onSaveSuccess();
        }
    }

    return (
        <div className={styles.container}>
            <div className="set_main">
                <input name="tpath" placeholder="Target Path" value={inputs.tpath} onChange={handleInputChange} />
                <input name="lpath" placeholder="Link Path" value={inputs.lpath} onChange={handleInputChange} />
                <input name="hpath" placeholder="Home Path" value={inputs.hpath} onChange={handleInputChange} />
                <button onClick={doSave}>Save</button>
            </div>
            <div className="set_help">
                <p>
                    <b>PortableRunner [Target Path] [Link Path] [Home Path]</b><br />
                    &nbsp;&nbsp;&nbsp;&nbsp;args:<br />
                    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<b>Target Path</b>: Specifies the physical drive and path that you want to assign to a virtual drive.<br />
                    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<b>Link Path</b>: Specifies the virtual drive and path to which you want to link the [Target Path].<br />
                    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<b>Home Path</b>: The subdirectory of the [Link Path], will be specified as the value of %HOME%, which defaults to [.home].<br />
                </p>
            </div>
        </div>
    );
}
