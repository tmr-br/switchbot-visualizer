import {
  TimestreamWriteClient,
  WriteRecordsCommand,
  WriteRecordsCommandInput,
} from "@aws-sdk/client-timestream-write"; // ES Modules import
import { globalConfig } from "./config";
import { SBEvent } from "./type";

const config = {}; // Optional TimestreamWriteClient configuration
const client = new TimestreamWriteClient(config);

const createEventInput = (event: SBEvent): WriteRecordsCommandInput | null => {
  if (
    event.eventType !== "changeReport" ||
    event.eventVersion !== "1" ||
    !event.context ||
    !event.context.deviceType ||
    !event.context.deviceMac ||
    !event.context.timeOfSample
  ) {
    // Invalid event
    return null;
  }

  switch (event.context.deviceType) {
    case "WoMeter":
      return {
        DatabaseName: globalConfig.env.timestreamDatabaseName,
        TableName: globalConfig.env.timesteamTableName,
        Records: [
          {
            Dimensions: [
              {
                Name: "deviceMac",
                Value: event.context.deviceMac,
              },
              {
                Name: "deviceType",
                Value: event.context.deviceType,
              },
            ],
            MeasureValueType: "MULTI",
            Time: String(event.context.timeOfSample),
            TimeUnit: "MILLISECONDS",
            Version: 1,
            MeasureValues: [
              {
                Name: "humidity",
                Value: String(event.context.humidity),
                Type: "DOUBLE",
              },
              {
                Name: "temperature",
                Value: String(event.context.temperature),
                Type: "DOUBLE",
              },
              {
                Name: "battery",
                Value: String(event.context.battery),
                Type: "DOUBLE",
              },
            ],
          },
        ],
      };
    case "MeterPro(CO2)":
      return {
        DatabaseName: globalConfig.env.timestreamDatabaseName,
        TableName: globalConfig.env.timesteamTableName,
        Records: [
          {
            Dimensions: [
              {
                Name: "deviceMac",
                Value: event.context.deviceMac,
              },
              {
                Name: "deviceType",
                Value: event.context.deviceType,
              },
            ],
            MeasureName: "sensorData",
            MeasureValueType: "MULTI",
            Time: String(event.context.timeOfSample),
            TimeUnit: "MILLISECONDS",
            Version: 1,
            MeasureValues: [
              {
                Name: "CO2",
                Value: String(event.context.CO2),
                Type: "DOUBLE",
              },
              {
                Name: "humidity",
                Value: String(event.context.humidity),
                Type: "DOUBLE",
              },
              {
                Name: "temperature",
                Value: String(event.context.temperature),
                Type: "DOUBLE",
              },
              {
                Name: "battery",
                Value: String(event.context.battery),
                Type: "DOUBLE",
              },
            ],
          },
        ],
      };
    case "WoHub2":
      return {
        DatabaseName: globalConfig.env.timestreamDatabaseName,
        TableName: globalConfig.env.timesteamTableName,
        Records: [
          {
            Dimensions: [
              {
                Name: "deviceMac",
                Value: event.context.deviceMac,
              },
              {
                Name: "deviceType",
                Value: event.context.deviceType,
              },
            ],
            MeasureValueType: "MULTI",
            Time: String(event.context.timeOfSample),
            TimeUnit: "MILLISECONDS",
            Version: 1,
            MeasureValues: [
              {
                Name: "humidity",
                Value: String(event.context.humidity),
                Type: "DOUBLE",
              },
              {
                Name: "temperature",
                Value: String(event.context.temperature),
                Type: "DOUBLE",
              },
              {
                Name: "lightLevel",
                Value: String(event.context.lightLevel),
                Type: "DOUBLE",
              },
            ],
          },
        ],
      };
    default:
      console.error(
        "Uninplemented device type:",
        (event.context as unknown as any).deviceType
      );
      return null;
  }
};

export const writeEvents = async (event: SBEvent) => {
  const input = createEventInput(event);
  if (input === null) {
    return;
  }
  const command = new WriteRecordsCommand(input);
  const response = await client.send(command);
  return response;
};
