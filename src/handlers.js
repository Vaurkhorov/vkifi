const { invoke } = window.__TAURI__.tauri;


export async function checkKifi(outputElement, pathElement) {
    pathElement.textContent = "Fetching..."
    return await invoke('meta', { path: pathElement.value })
        .then(() => {
            let repoPath = pathElement.value;
            outputElement.textContent = "Found kifi at " + repoPath;
            return repoPath;
        })
        .catch(err => {
            outputElement.textContent = "Error finding repository:\n" + err;
            return null;
        });
}
  
export async function preview(outputElement, repoPath) {
    await invoke('preview', { path: repoPath })
        .then(result => {
            if (result) {
                outputElement.textContent = result;
            }
        })
        .catch(err => {
            outputElement.textContent = "Error while getting preview at " + repoPath + ":\n" + err;
        });
}

export async function meta(outputElement, repoPath) {
    await invoke('meta', { path: repoPath })
        .then(result => {
            if (result) {
                outputElement.textContent = result;
            }
        })
        .catch(err => {
            outputElement.textContent = "Error while getting metadata from " + repoPath + ":\n" + err;
        });
}