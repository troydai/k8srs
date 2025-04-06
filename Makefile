.PHONY: kind-create kind-delete kind-logs kind-status

# Create a new Kind cluster
kind-create:
	@echo "Creating Kind cluster..."
	kind create cluster --name k8srs

# Delete the Kind cluster
kind-delete:
	@echo "Deleting Kind cluster..."
	kind delete cluster --name k8srs

# Show cluster logs
kind-logs:
	@echo "Showing Kind cluster logs..."
	kind export logs --name k8srs

# Show cluster status
kind-status:
	@echo "Showing Kind cluster status..."
	kubectl cluster-info --context kind-k8srs
	kubectl get nodes

# Deploy httpbin application
deploy-httpbin:
	@echo "Deploying httpbin application..."
	kubectl apply -f k8s/httpbin.yaml
	@echo "Waiting for deployment to be ready..."
	kubectl wait --for=condition=available --timeout=60s deployment/httpbin

# Delete httpbin application
delete-httpbin:
	@echo "Deleting httpbin application..."
	kubectl delete -f k8s/httpbin.yaml

# Port forward to httpbin service
port-forward-httpbin:
	@echo "Port forwarding to httpbin service..."
	kubectl port-forward service/httpbin 8080:80 