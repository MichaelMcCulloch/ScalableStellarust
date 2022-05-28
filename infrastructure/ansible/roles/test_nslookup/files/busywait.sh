#/bin/sh


unready_pods=`kubectl get pods --all-namespaces | awk '{ print $3}' | grep 0\/`
if [ -z "$unready_pods" ]
then
    exit 0
fi

prev=`kubectl get pods --all-namespaces | awk '{ print $1,$2,$3,$4}'`
sleep 15s
curr=`kubectl get pods --all-namespaces | awk '{ print $1,$2,$3,$4}'`
difference=`diff <(echo "$prev") <(echo "$curr")`

unready_pods=`kubectl get pods --all-namespaces | awk '{ print $3}' | grep 0\/`
while [ -n "$difference" ] || [ -n "$unready_pods" ]; do
    prev=$curr
    sleep 15s
    curr=`kubectl get pods --all-namespaces | awk '{ print $1,$2,$3,$4}'`
    difference=`diff <(echo "$prev") <(echo "$curr")`
    unready_pods=`kubectl get pods --all-namespaces | awk '{ print $3}' | grep 0\/`
done
exit 0