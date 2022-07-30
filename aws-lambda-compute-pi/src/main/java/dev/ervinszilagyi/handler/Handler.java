package dev.ervinszilagyi.handler;

import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyRequestEvent;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Handler implements RequestHandler<APIGatewayProxyRequestEvent, String> {
    private static final Logger logger = LoggerFactory.getLogger(Handler.class);
    private static final Gson gson = new GsonBuilder().setPrettyPrinting().create();

    @Override
    public String handleRequest(APIGatewayProxyRequestEvent event, Context context) {
        PIGenerator piGenerator = new PIGenerator();
        return IntStream.generate(piGenerator::getDigit)
                .limit(10_000)
                .mapToObj(String::valueOf)
                .collect(Collectors.joining());
    }
}
