export interface SBEvent {
  eventType: string;
  eventVersion: string;
  context:
    | SBMeterProCO2EventContext
    | SBWoMeterEventContext
    | SBWoHub2EventContext;
}

export interface SBWoMeterEventContext {
  battery: number;
  deviceMac: string;
  deviceType: "WoMeter";
  humidity: number;
  scale: string;
  temperature: number;
  timeOfSample: number;
}

export interface SBMeterProCO2EventContext {
  CO2: number;
  battery: number;
  deviceMac: string;
  deviceType: "MeterPro(CO2)";
  humidity: number;
  scale: string;
  temperature: number;
  timeOfSample: number;
}

export interface SBWoHub2EventContext {
  deviceMac: string;
  deviceType: "WoHub2";
  humidity: number;
  lightLevel: number;
  scale: string;
  temperature: number;
  timeOfSample: number;
}

// {
//     "eventType": "changeReport",
//     "eventVersion": "1",
//     "context": {
//         "CO2": 1395,
//         "battery": 100,
//         "deviceMac": "B0E9FE548DD9",
//         "deviceType": "MeterPro(CO2)",
//         "humidity": 31,
//         "scale": "CELSIUS",
//         "temperature": 26.1,
//         "timeOfSample": 1733809095937
//     }
// }

export interface SBWoMeterEvent extends SBEvent {
  eventType: "changeReport";
  eventVersion: "1";
  context: SBWoMeterEventContext;
}

// {
//     "eventType": "changeReport",
//     "eventVersion": "1",
//     "context": {
//         "battery": 100,
//         "deviceMac": "CA5FC3064551",
//         "deviceType": "WoMeter",
//         "humidity": 32,
//         "scale": "CELSIUS",
//         "temperature": 21.8,
//         "timeOfSample": 1733807344651
//     }
// }

export interface SBMeterProCO2Event extends SBEvent {
  eventType: "changeReport";
  eventVersion: "1";
  context: SBMeterProCO2EventContext;
}

// {
//     "eventType": "changeReport",
//     "eventVersion": "1",
//     "context": {
//         "deviceMac": "E625C2243B40",
//         "deviceType": "WoHub2",
//         "humidity": 36,
//         "lightLevel": 11,
//         "scale": "CELSIUS",
//         "temperature": 22.6,
//         "timeOfSample": 1733813537934
//     }
// }

export interface SBWoHub2Event extends SBEvent {
  eventType: "changeReport";
  eventVersion: "1";
  context: SBWoHub2EventContext;
}
