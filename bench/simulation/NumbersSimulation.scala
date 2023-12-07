import scala.concurrent.duration._
import scala.collection.mutable.ListBuffer
import scala.util.Random
import scala.math._

import io.gatling.core.Predef._
import io.gatling.http.Predef._
import io.gatling.core.structure.ScenarioBuilder

class NumbersSimulation extends Simulation {
  val random = new Random
   val httpProtocol = http
    .baseUrl("http://localhost")
  val data = List(1, 2, 3, 4, 5).map(value => Map("value" -> pow(10, value).toInt))
  def viewSim(scenarioName: String, port: String): ScenarioBuilder = {
    return scenario(scenarioName)
      .feed(Iterator.continually(data).flatten)
      .exec(
        http(s"$scenarioName view_numbers")
          .get(s":$port/numbers?limit=#{value}")
          .header("content-type", "application/json")
      )
  }
  val nodeSim = viewSim("NodeSimulation", "8082")
  val rustSim = viewSim("RustSimulation", "8081")
  val createNumbers = scenario("Populating Database")
    .exec(
        http("create_numbers")
          .post(":8082/numbers")
          .body(StringBody("{\"value\":\"1000000\"}"))
          .header("content-type", "application/json")
      )
  setUp(
    createNumbers.inject(atOnceUsers(1)).andThen(
      rustSim.inject(
        constantConcurrentUsers(20).during(30.seconds)
      ).andThen(
        nodeSim.inject(
          constantConcurrentUsers(20).during(30.seconds)
        ) 
      )
    )
  ).protocols(httpProtocol)
}