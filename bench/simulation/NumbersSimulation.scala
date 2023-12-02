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
  val data = List(1, 2, 3, 4, 5).map(value => Map("value" -> value))
  def sim(scenarioName: String, port: String): ScenarioBuilder = {
    return scenario(scenarioName)
      .feed(Iterator.continually(data).flatten)
      .exec(
        http("create_numbers")
          .post(s":$port/numbers")
          .body(StringBody("{\"value\":\"#{value}\"}"))
          .header("content-type", "application/json")
      )
      .exec(
        http("view_numbers")
          .get(s":$port/numbers")
          .header("content-type", "application/json")
      )
  }
  val nodeSim = sim("NodeSimulation", "8082")
  val rustSim = sim("RustSimulation", "8082")

  setUp(
    rustSim.inject(
      constantConcurrentUsers(20).during(1.minutes)
    ).andThen(
      nodeSim.inject(
        constantConcurrentUsers(20).during(1.minutes)
      ) 
    )
  ).protocols(httpProtocol)
}