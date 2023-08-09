// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "health", input: never, result: string } | 
        { key: "user.getUser", input: never, result: { id: string; name: string | null; username: string | null; email: string; password: string; devices: Device[] } },
    mutations: 
        { key: "auth.login", input: LoginData, result: string } | 
        { key: "auth.register", input: LoginData, result: string } | 
        { key: "user.addDevice", input: DeviceInfo, result: string },
    subscriptions: never
};

export type LoginData = { email: string; username: string; password: string }

export type DeviceInfo = { name: string; user_id: string; device_type: string }

export type Device = { id: string; type: DeviceType; name: string; userId: string }

export type DeviceType = "Desktop" | "Phone"
