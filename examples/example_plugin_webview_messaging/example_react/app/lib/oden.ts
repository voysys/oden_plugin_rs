export function getOrCreateOdenLayoutClient() {
    if (typeof window === 'undefined') return null;
    if (!(window as any).odenLayoutClient) {

        (window as any).odenLayoutClient = new OdenLayoutClient();
    }
    return (window as any).odenLayoutClient as InstanceType<any>;
}
