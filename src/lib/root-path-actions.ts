export const OPEN_ADD_ROOT_PATH_EVENT = 'gitradar:open-add-root-path';
export const OPEN_ADD_ROOT_PATH_FLAG = 'gitradar-open-add-root-path';

export function requestAddRootPathPopover({ persist = false }: { persist?: boolean } = {}) {
  if (persist) {
    sessionStorage.setItem(OPEN_ADD_ROOT_PATH_FLAG, '1');
  }

  window.dispatchEvent(new Event(OPEN_ADD_ROOT_PATH_EVENT));
}

export function consumeAddRootPathPopoverRequest() {
  const shouldOpen = sessionStorage.getItem(OPEN_ADD_ROOT_PATH_FLAG) === '1';

  if (shouldOpen) {
    sessionStorage.removeItem(OPEN_ADD_ROOT_PATH_FLAG);
  }

  return shouldOpen;
}
