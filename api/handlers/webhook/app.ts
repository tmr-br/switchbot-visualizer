import { APIGatewayProxyEvent, APIGatewayProxyResult } from "aws-lambda";
import { writeEvents } from "./timestream";

export const lambdaHandler = async (
  event: APIGatewayProxyEvent
): Promise<APIGatewayProxyResult> => {
  console.log("body:", event.body);
  let response: APIGatewayProxyResult;
  try {
    const body = JSON.parse(event.body || "{}");
    await writeEvents(body);
    response = {
      statusCode: 200,
      body: JSON.stringify({
        message: "hello world",
      }),
    };
  } catch (err: unknown) {
    console.error(err);
    response = {
      statusCode: 500,
      body: JSON.stringify({
        message: err instanceof Error ? err.message : "some error happened",
      }),
    };
  }
  return response;
};
